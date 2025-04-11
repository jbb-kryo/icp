import { useContext } from "react";
import { PlatformContext } from "../context/PlatformContext";

export const usePlatforms = () => useContext(PlatformContext);