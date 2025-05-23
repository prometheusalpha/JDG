import { Moon, Sun } from "lucide-react";

import { useTheme } from "@/components/theme-provider";
import { Switch } from "@/components/ui/switch"; // Import Switch

export function ModeToggle() {
  // Assuming useTheme provides the current theme value
  const { theme, setTheme } = useTheme();

  // Determine if dark mode is currently active
  const isDarkMode = theme === 'dark';

  return (
    <div className="flex items-center space-x-2"> {/* Container for switch and icon */}
      <Switch
        id="theme-toggle" // Add an ID for accessibility
        checked={isDarkMode} // Set checked state based on current theme
        onCheckedChange={(checked) => setTheme(checked ? "dark" : "light")} // Toggle between dark and light
        aria-label="Toggle theme" // Add aria-label for accessibility
      />
      {/* Display the appropriate icon based on the current theme */}
      {isDarkMode ? (
        <Moon className="h-[1.2rem] w-[1.2rem]" />
      ) : (
        <Sun className="h-[1.2rem] w-[1.2rem]" />
      )}
    </div>
  );
}
