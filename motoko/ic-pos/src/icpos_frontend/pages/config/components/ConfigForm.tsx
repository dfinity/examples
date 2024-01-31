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
import { Switch } from "../../../components/ui/switch";
import toast from "react-hot-toast";
import { useForm } from "react-hook-form";
import { useIcPos } from "../../../canisters/ic-pos/hooks/useIcPos";
import { useNavigate } from "@tanstack/router";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";

const MerchantSchema = z.object({
  name: z.string().min(2, {
    message: "Username must be at least 2 characters.",
  }),
  email_notifications: z.boolean(),
  email_address: z.string().email().optional().or(z.literal("")),
  phone_notifications: z.boolean(),
  phone_number: z.string().min(10).optional().or(z.literal("")),
});

type MerchantSchemaType = z.infer<typeof MerchantSchema>;

export default function ConfigForm() {
  const { merchantState, updateMerchant } = useIcPos();
  const navigate = useNavigate();

  const form = useForm<MerchantSchemaType>({
    resolver: zodResolver(MerchantSchema),
    defaultValues: {
      name: "",
      email_notifications: false,
      email_address: "",
      phone_notifications: false,
      phone_number: "",
    },
  });

  React.useEffect(() => {
    if (!merchantState.initialized || !merchantState.merchant) return;
    form.setValue("name", merchantState.merchant.name);
    form.setValue(
      "email_notifications",
      merchantState.merchant.email_notifications
    );
    form.setValue("email_address", merchantState.merchant.email_address);
    form.setValue(
      "phone_notifications",
      merchantState.merchant.phone_notifications
    );
    form.setValue("phone_number", merchantState.merchant.phone_number);
  }, [form, merchantState]);

  async function onSubmit(values: MerchantSchemaType) {
    if (values.email_notifications && !values.email_address) {
      return form.setError("email_address", {
        message: "Email address is required.",
      });
    }
    if (values.phone_notifications && !values.phone_number) {
      return form.setError("phone_number", {
        message: "Phone number is required.",
      });
    }
    const response = await updateMerchant({
      ...values,
      email_address: values.email_address ? values.email_address : "",
      phone_number: values.phone_number ? values.phone_number : "",
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
        <div className="flex flex-col p-5 space-y-3 border rounded-lg">
          <FormField
            control={form.control}
            name="email_notifications"
            render={({ field }) => (
              <FormItem>
                <div className="flex flex-row items-center justify-between">
                  <FormLabel>Email notifications</FormLabel>
                  <FormControl>
                    <Switch
                      checked={field.value}
                      onCheckedChange={field.onChange}
                    />
                  </FormControl>
                </div>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="email_address"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Email address</FormLabel>
                <FormControl>
                  <Input
                    placeholder="info@jillsvegetables.com"
                    disabled={
                      form.getValues().email_notifications === true
                        ? false
                        : true
                    }
                    {...field}
                  />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
        </div>

        <div className="flex flex-col p-5 space-y-3 border rounded-lg">
          <FormField
            control={form.control}
            name="phone_notifications"
            render={({ field }) => (
              <FormItem>
                <div className="flex flex-row items-center justify-between">
                  <FormLabel>Phone notifications</FormLabel>
                  <FormControl>
                    <Switch
                      checked={field.value}
                      onCheckedChange={field.onChange}
                    />
                  </FormControl>
                </div>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="phone_number"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Phone number</FormLabel>
                <FormControl>
                  <Input
                    placeholder="+1-555-123 456"
                    disabled={
                      form.getValues().phone_notifications === true
                        ? false
                        : true
                    }
                    {...field}
                  />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
        </div>
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
