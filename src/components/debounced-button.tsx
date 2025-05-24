import { useEffect, useState } from "react";
import { Button } from "./ui/button";

interface DebounceButtonProps {
  func: () => void;
  delay?: number;
  title?: string;
}

export const DebounceButton = ({
  func,
  delay = 2000,
  title = "Copy",
  ...props
}: DebounceButtonProps) => {
  const [hasDone, setHasDone] = useState(false);

  useEffect(() => {
    setTimeout(() => {
      setHasDone(false);
    }, delay);
  }, [hasDone]);

  return (
    <Button
      variant="outline"
      size="sm"
      onClick={() => {
        func();
        setHasDone(true);
      }}
      disabled={hasDone}
      {...props}
    >
      {hasDone ? "Done!" : title}
    </Button>
  );
};
