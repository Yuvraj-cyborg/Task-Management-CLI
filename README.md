# Minor-project

## Introduction
Minor-project is a versatile command-line interface (CLI) application designed to help users manage their tasks efficiently. It offers a wide range of functionalities from basic task management to advanced features like cloud synchronization and sharing capabilities among users.

## Features

### Task Management
- **View Tasks**: List tasks with various filters.
  - `todo show`: Shows all the list names.
  - `todo show -a`: Shows all the list names along with the items.
  - `todo show -c`: Shows all the completed items of all lists.
  - `todo show -i`: Shows all the incomplete items of all lists.
  - `todo show <list_name>`: Shows all the items of that list.
  - `todo show <list_name> -c`: Shows all the completed items of that list.
  - `todo show <list_name> -i`: Shows all the incomplete items of that list.

- **Add Tasks**: Add items to your lists.
  - `todo add <list_name> <item>`: Adds the item to that list.

- **Complete/Incomplete Tasks**: Mark tasks as completed or incomplete.
  - `todo complete <list_name> <item_number>`: Marks an item as completed.
  - `todo incomplete <list_name> <item_number>`: Marks an item as incomplete.

- **Remove Tasks**: Remove tasks or lists.
  - `todo remove`: Removes all lists.
  - `todo remove <list_name>`: Removes that particular list.
  - `todo remove <list_name> <item_number>`: Removes that item from the list.

### Optional Features
- **User Authentication**: Manage user sessions.
  - `todo login`: Logs in the user.
  - `todo logout`: Logs out the user.

- **Cloud Synchronization**: Keep your tasks synced across devices.
  - `todo push`: Syncs local changes with the cloud.
  - `todo pull`: Fetches updates from the cloud.

- **Sharing and Notifications**: Collaborate on tasks and manage notifications.
  - `todo share <list_name> <email>`: Shares the list with another user.
  - `todo unshare <list_name> <email>`: Revokes access from a shared user.
  - `todo show notifications`: Displays all notifications for the user.

## Usage

To start managing your tasks, use the `todo` command followed by the action you want to perform. For example, to add a new task, you would use:

```bash
todo add "Grocery List" "Buy milk"
