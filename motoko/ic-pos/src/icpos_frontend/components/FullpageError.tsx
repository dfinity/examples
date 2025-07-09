import { Ban } from "lucide-react";
import MainSection from "./MainSection";

export default function FullpageError() {
  return (
    <MainSection>
      <div className="flex flex-col items-center justify-center min-h-screen space-y-5 md:min-h-0 md:h-[750px]">
        <Ban className="w-24 h-24 text-red-500" />
      </div>
    </MainSection>
  );
}
