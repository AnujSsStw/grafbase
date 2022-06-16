import {TodoListFragment, useTodoListDeleteMutation} from "graphql/schema";
import { useMemo } from "react";
import TodoListCreateTodo from "components/todo-list.create-todo";
import TodoListTodo from "components/todo-list.todo";

const TodoList = (props: TodoListFragment) => {
  const { id, title, todos } = props;
  const contextDeleteTodoList = useMemo(
    () => ({ additionalTypenames: ["TodoList"] }),
    []
  );
  const [{ fetching }, todoListDelete] = useTodoListDeleteMutation();

  return (
    <div className="space-y-2 flex-1">
      <div className="relative rounded-lg border border-gray-200 p-4">
        <h2 className="font-semibold">{title}</h2>
        <button
          onClick={() => todoListDelete({ id }, contextDeleteTodoList)}
          className="absolute text-gray-400 flex items-center justify-center -right-2 -top-2 rounded-full h-6 w-6 p-2 border border-gray-200 text-sm bg-white hover:bg-red-400 hover:text-white transition"
        >
            { fetching ? '...' : 'x'}
        </button>
      </div>
      <div className="space-y-1  ">
        {todos?.map(
          (todo) => !!todo && <TodoListTodo key={todo.id} {...todo} />
        )}
      </div>
      <TodoListCreateTodo todoListId={id} />
    </div>
  );
};

export default TodoList;
