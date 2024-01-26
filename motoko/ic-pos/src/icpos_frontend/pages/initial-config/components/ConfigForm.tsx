// import * as z from "zod";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "../../../components/ui/form";

import { Button } from "../../../components/ui/button";
import { Input } from "../../../components/ui/input";
import { Loader2 } from "lucide-react";
import React from "react";
import toast from "react-hot-toast";
import { useForm } from "react-hook-form";
import { useIcPos } from "../../../canisters/ic-pos/hooks/useIcPos";
import { useNavigate } from "@tanstack/router";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";

const MerchantSchema = z.object({
  name: z.string().min(2, {
    message: "Store name must be at least 2 characters.",
  }),
});

type MerchantSchemaType = z.infer<typeof MerchantSchema>;

export default function ConfigForm() {
  const { merchantState, updateMerchant } = useIcPos();
  const navigate = useNavigate();

  const form = useForm<MerchantSchemaType>({
    resolver: zodResolver(MerchantSchema),
    defaultValues: {
      name: "",
    },
  });

  React.useEffect(() => {
    if (!merchantState.initialized || !merchantState.merchant) return;
    form.setValue("name", merchantState.merchant.name);
  }, [form, merchantState]);

  async function onSubmit(values: MerchantSchemaType) {
    const response = await updateMerchant({
      ...values,
      email_address: merchantState.merchant?.email_address || "",
      email_notifications: merchantState.merchant?.email_notifications || false,
      phone_number: merchantState.merchant?.phone_number || "",
      phone_notifications: merchantState.merchant?.phone_notifications || false,
    });

    if (response && response.status === 200) {
      toast.success("Merchant settings updated.");
      navigate({ to: "/merchant" });
    } else {
      response?.error_text &&
        toast.error(response?.error_text[0] || "An error occurred.");
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
