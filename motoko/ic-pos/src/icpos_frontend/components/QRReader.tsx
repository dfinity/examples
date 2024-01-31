import { Button } from "./ui/button";
import { Result, useZxing } from "react-zxing";

type QrReaderProps = {
  onResult: (result: Result) => void;
  setVisible: (visible: boolean) => void;
  visible: boolean;
};

export default function QRReader(props: QrReaderProps) {
  const { ref } = useZxing({
    onResult: props.onResult,
    paused: !props.visible,
    constraints: {
      video: {
        facingMode: "environment",
      },
    },
  });

  if (!props.visible) return null;
  return (
    <div className="w-full">
      <div className="flex items-center justify-between p-5 mb-5 border rounded-lg">
        <video ref={ref} className="w-full h-72" />
      </div>

      <Button className="w-full m-0" onClick={() => props.setVisible(false)}>
        Cancel
      </Button>
    </div>
  );
}
