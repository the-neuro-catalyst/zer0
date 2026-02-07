import * as React from "react";

export interface ToastProps {
  id: string;
  title?: React.ReactNode;
  description?: React.ReactNode;
  action?: React.ReactElement;
  open?: boolean;
  onOpenChange?: (open: boolean) => void;
}

interface State {
  toasts: ToastProps[];
}

const listeners: Array<(state: State) => void> = [];
const memoryState: State = { toasts: [] };

export function useToast() {
  const [state, setState] = React.useState<State>(memoryState);

  React.useEffect(() => {
    listeners.push(setState);
    return () => {
      const index = listeners.indexOf(setState);
      if (index > -1) listeners.splice(index, 1);
    };
  }, []);

  return {
    ...state,
    toast: (_props: Omit<ToastProps, "id">) => {
      const id = Math.random().toString(36).substring(2, 9);
      return {
        id,
        dismiss: () => {},
        update: (_p: ToastProps) => {},
      };
    },
    dismiss: (_id?: string) => {},
  };
}

export const toast = (_props: Omit<ToastProps, "id">) => ({
  id: "1",
  dismiss: () => {},
  update: (_p: ToastProps) => {},
});