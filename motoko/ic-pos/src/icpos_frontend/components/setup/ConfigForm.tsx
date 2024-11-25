// import * as z from "zod";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";

import { Loader2 } from "lucide-react";
import { useEffect } from "react";
import toast from "react-hot-toast";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import useMerchant from "@/hooks/useMerchant";
import { Button } from "../ui/button";
import { Input } from "../ui/input";
import { useIcPosActor } from "@/actors";
import { useNavigate } from "@tanstack/react-router";
import { queryClient } from "@/main";

const MerchantSchema = z.object({
  name: z.string().min(2, {
    message: "Store name must be at least 2 characters.",
  }),
});

type MerchantSchemaType = z.infer<typeof MerchantSchema>;

export default function ConfigForm() {
  const { data: merchant } = useMerchant();
  const { actor: pos } = useIcPosActor();
  const navigate = useNavigate();

  const form = useForm<MerchantSchemaType>({
    resolver: zodResolver(MerchantSchema),
    defaultValues: {
      name: "",
    },
  });

  useEffect(() => {
    if (!merchant) return
    form.setValue("name", merchant.name);
  }, [merchant]);

  async function onSubmit(values: MerchantSchemaType) {
    const response = await pos?.updateMerchant({
      ...values,
      email_address: merchant?.email_address || "",
      email_notifications: merchant?.email_notifications || false,
      phone_number: merchant?.phone_number || "",
      phone_notifications: merchant?.phone_notifications || false,
    });

    if (response && response.status === 200) {
      toast.success("Merchant settings updated.");
      queryClient.invalidateQueries({ queryKey: ['merchant'] })
      setTimeout(() => {
        navigate({ to: "/" });
      }, 500);
    } else {
      if (response?.error_text) {
        toast.error(response?.error_text[0] || "An error occurred.");
      }
    }
  }

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className="w-full space-y-5">
        <FormField
          control={form.control}
          name="name"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Store Name</FormLabel>
              <FormControl>
                <Input placeholder="Jill's Vegetables" {...field} />
              </FormControl>
              <FormDescription>
                This is the name of your store as displayed to customers.
              </FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />

        {form.formState.isSubmitting ? (
          <Button disabled className="w-full">
            <Loader2 className="w-4 h-4 mr-2 animate-spin" />
            Saving
          </Button>
        ) : (
          <Button type="submit" className="w-full">
            Save
          </Button>
        )}
      </form>
    </Form>
  );
}
