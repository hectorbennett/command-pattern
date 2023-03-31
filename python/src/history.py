class History:

    def __init__(self):
        # A log of all the commands in their execution order
        self.history = []

        # The position in the history we want to execute to
        self.revision = 0

    def append(self, command):
        # Destroy anything ahead of the current revision
        self.history = self.history[0:self.revision]
        
        # Add a command to the history
        self.history.append(command)

        # move forward one step in the history
        self.revision += 1

        # execute the function
        command.execute()
    
    def undo(self):
        if not self.history:
            return

        # Move the cursor back 1
        self.revision = max(0, self.revision - 1)

        # undo the current command
        self.history[self.revision].rollback()

    def redo(self):
        if self.revision == len(self.history):
            return

        # redo the current command
        self.history[self.revision].execute()

        # Move forwards (again) to where we were in history
        self.revision += 1
