import { createBrowserRouter } from "react-router-dom";
import Homepage from "./pages/homepage";
import SettingsPage from "./pages/setting";
import ProfilePage from "./pages/profile.tsx";
import "./App.css";

export const router = createBrowserRouter([
  {
    path: "/",
    element: <Homepage />,
  },

  {
    path: "/settings",
    element: <SettingsPage />,
  },

  {
    path: "/profile",
    element: <ProfilePage />,
  },
]);
