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
import React from "react";
import toast from "react-hot-toast";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import useMerchant from "@/hooks/useMerchant";
import useUpdateMerchant from "@/hooks/useUpdateMerchant";
import { useNavigate } from "@tanstack/react-router";
import { Input } from "../ui/input";
import { Button } from "../ui/button";
import { Switch } from "../ui/switch";

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
  const navigate = useNavigate();
  const { data: merchant } = useMerchant();
  const { mutateAsync: updateMerchant } = useUpdateMerchant()

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
    if (!merchant) return;
    form.setValue("name", merchant.name);
    form.setValue(
      "email_notifications",
      merchant.email_notifications
    );
    form.setValue("email_address", merchant.email_address);
    form.setValue(
      "phone_notifications",
      merchant.phone_notifications
    );
    form.setValue("phone_number", merchant.phone_number);
  }, [form, merchant]);

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
    try {
      await updateMerchant({
        ...values,
        email_address: values.email_address ? values.email_address : "",
        phone_number: values.phone_number ? values.phone_number : "",
      });
      navigate({ to: "/" });
      toast.success("Merchant updated");
    } catch {
      toast.error("Couldn't update merchant");
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
