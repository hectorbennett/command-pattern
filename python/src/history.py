class History:
    def __init__(self):
        # A log of all the commands in their execution order
        self.history = []

        # Where we have executed up to so far
        self.cursor = 0

        # The position in the history we want to execute to
        self.revision = 0

    def append(self, command):
        # Destroy anything ahead of the current revision
        self.history = self.history[0 : self.revision]

        # Add a command to the history
        self.history.append(command)

        # move forward one step in the history
        self.revision += 1

    def execute(self):
        # execute all the methods that have not yet been executed
        for i in range(self.cursor, self.revision):
            self.history[i].execute()
        self.cursor = self.revision

    def undo(self):
        if not self.history:
            return

        # Move the cursor back 1
        self.revision = max(0, self.revision - 1)

        # undo the current command
        self.history[self.revision].rollback()

        self.cursor = self.revision

    def redo(self):
        if self.revision == len(self.history):
            return

        # redo the current command
        self.history[self.revision].execute()

        # Move forwards (again) to where we were in history
        self.revision += 1

        self.cursor = self.revision
