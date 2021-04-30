mod HSMBase{
// The structure returned by update.
    mod UpdateResult{
     actions
     transition
     level
    
    fn getActions() -> Action[]{
     return []
}
     fn update() -> UpdateResult{
     UpdateResult result = new UpdateResult()
     result.actions = getActions()
     result.transition = null
     result.level = 0
     return result
}
     fn getStates() -> State[] {} // Unimplemented in base class.
    }
}
