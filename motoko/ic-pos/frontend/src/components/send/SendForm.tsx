import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form"

import { Loader2 } from "lucide-react";
import { Principal } from "@icp-sdk/core/principal";
import { toast } from "react-hot-toast";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import { convertToBigInt } from "@/utils/convertToBigInt";
import { Input } from "../ui/input";
import { Button } from "../ui/button";
import { useNavigate } from "@tanstack/react-router";
import useTokenBalance from "@/hooks/useTokenBalance";
import { useIcrcLedger } from "@/actors";
import { queryClient } from "@/main";

type SendFormProps = {
  principal: string;
  amount: string;
};

const SendSchema = z.object({
  to: z.string(),
  amount: z.string(),
});

type SendSchemaType = z.infer<typeof SendSchema>;

export default function SendForm({ principal, amount }: SendFormProps) {
  const navigate = useNavigate();
  const { data: balance } = useTokenBalance();
  const ledgerCanister = useIcrcLedger();

  const form = useForm<SendSchemaType>({
    resolver: zodResolver(SendSchema),
    defaultValues: {
      to: principal,
      amount
    },
  });

  async function onSubmit(values: SendSchemaType) {
    let toPrincipal: Principal;
    try {
      toPrincipal = Principal.fromText(values.to);
    } catch {
      return form.setError("to", {
        message: "Invalid principal address",
      });
    }

    const amount = Number.parseFloat(values.amount);
    if (Number.isNaN(amount)) {
      return form.setError("amount", {
        message: "Amount must be a number.",
      });
    }
    if (amount <= 0) {
      return form.setError("amount", {
        message: "Amount must be greater than 0.",
      });
    }
    let amountBigInt: bigint;
    try {
      amountBigInt = convertToBigInt(amount);
    } catch (error) {
      return form.setError("amount", {
        message: (error as Error).message,
      });
    }
    if (balance && amountBigInt > balance) {
      return form.setError("amount", {
        message: "Amount exceeds balance.",
      });
    }

    try {
      const response = await ledgerCanister.transfer({
        to: {
          owner: toPrincipal,
          subaccount: [],
        },
        amount: amountBigInt,
      });

      if (response !== undefined) {
        toast.success("Transfer successful.");
        queryClient.invalidateQueries({ queryKey: ['balance'] });
        queryClient.invalidateQueries({ queryKey: ['latest_transactions'] });
        setTimeout(() => {
          navigate({ to: "/" });
        }, 500);
      } else {
        toast.error("An error occurred.");
      }
    } catch (error) {
      toast.error((error as Error).message);
      console.error(error);
    }
  }

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className="w-full space-y-5">
        <div className="flex flex-col w-full p-5 space-y-3 border rounded-lg">
          <FormField
            control={form.control}
            name="to"
            render={({ field }) => (
              <FormItem>
                <FormLabel>To</FormLabel>
                <FormControl>
                  <Input placeholder="p6s35-k6zg4..." {...field} />
                </FormControl>
                <FormDescription>
                  Enter the principal address of the recipient.
                </FormDescription>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="amount"
            render={({ field }) => (
              <FormItem>
                <div className="flex flex-row items-center justify-between">
                  <FormLabel className="pr-2">Amount</FormLabel>
                  <FormControl>
                    <Input type="number" {...field} />
                  </FormControl>
                </div>
                <FormDescription>
                  Transaction Fee: 0.0000001 ckBTC
                </FormDescription>
                <FormMessage />
              </FormItem>
            )}
          />
        </div>

        {form.formState.isSubmitting ? (
          <Button disabled className="w-full">
            <Loader2 className="w-4 h-4 mr-2 animate-spin" />
            Sending
          </Button>
        ) : (
          <Button type="submit" className="w-full">
            Send
          </Button>
        )}
      </form>
    </Form>
  );
}
