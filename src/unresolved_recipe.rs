use crate::common::*;

pub(crate) type UnresolvedRecipe<'src> = Recipe<'src, UnresolvedDependency<'src>>;

impl<'src> UnresolvedRecipe<'src> {
  pub(crate) fn resolve(
    self,
    resolved: Vec<Rc<Recipe<'src>>>,
    subsequent: Option<Rc<Recipe<'src>>>,
  ) -> CompilationResult<'src, Recipe<'src>> {
    assert_eq!(
      self.dependencies.len(),
      resolved.len(),
      "UnresolvedRecipe::resolve: dependency count not equal to resolved count: {} != {}",
      self.dependencies.len(),
      resolved.len()
    );

    for (unresolved, resolved) in self.dependencies.iter().zip(&resolved) {
      assert_eq!(unresolved.recipe.lexeme(), resolved.name.lexeme());
      if !resolved
        .argument_range()
        .contains(&unresolved.arguments.len())
      {
        return Err(unresolved.recipe.error(
          CompilationErrorKind::DependencyArgumentCountMismatch {
            dependency: unresolved.recipe.lexeme(),
            found:      unresolved.arguments.len(),
            min:        resolved.min_arguments(),
            max:        resolved.max_arguments(),
          },
        ));
      }
    }

    let dependencies = self
      .dependencies
      .into_iter()
      .zip(resolved)
      .map(|(unresolved, resolved)| Dependency {
        recipe:    resolved,
        arguments: unresolved.arguments,
      })
      .collect();

    let subsequent = match (self.subsequent, subsequent) {
      (Some(unresolved), Some(resolved)) => Some(Dependency {
        recipe:    resolved,
        arguments: unresolved.arguments,
      }),
      (None, None) => None,
      (None, Some(_)) =>
        panic!("UnresolvedRecipe::resolve: resolved subsequent passed when none expected"),
      (Some(_), None) => panic!("UnresolvedRecipe::resolve: resolved subsequent expected"),
    };

    Ok(Recipe {
      body: self.body,
      doc: self.doc,
      name: self.name,
      parameters: self.parameters,
      private: self.private,
      quiet: self.quiet,
      shebang: self.shebang,
      dependencies,
      subsequent,
    })
  }
}
