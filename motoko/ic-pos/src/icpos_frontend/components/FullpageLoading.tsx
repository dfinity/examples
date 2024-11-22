import { Loader2 } from "lucide-react";
import MainSection from "./MainSection";

export default function FullpageLoading() {
  return (
    <div className="min-h-screen bg-gradient-to-br from-gray-200 to-gray-500 md:flex md:flex-col md:items-center md:justify-center print-bg">

      <MainSection>
        <div className="flex flex-col items-center justify-center min-h-screen space-y-5 md:min-h-0 md:h-[750px]">
          <Loader2 className="w-24 h-24 animate-spin" />
        </div>
      </MainSection>
    </div>
  );
}
