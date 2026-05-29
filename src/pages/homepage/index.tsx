import { ModeToggle } from "@/components/mode-toggle";

export default function Homepage() {
  return (
    <div className="flex h-screen flex-col items-center justify-center gap-4 bg-background text-foreground">
      <h1 className="text-3xl font-bold">AI Interview Assistant</h1>
      <div className="flex h-screen items-center justify-center">
        <ModeToggle />
      </div>
    </div>
  );
}
