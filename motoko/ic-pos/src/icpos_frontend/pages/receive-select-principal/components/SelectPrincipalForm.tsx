import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { Button } from "../../../components/ui/button";
import {
  Form,
  FormField,
  FormItem,
  FormLabel,
  FormControl,
  FormDescription,
  FormMessage,
} from "../../../components/ui/form";
import { Input } from "../../../components/ui/input";
import { Principal } from "@dfinity/principal";
import { useNavigate } from "@tanstack/router";

type SelectPrincipalFormProps = {
  principal: string;
};

const SelectPrincipalSchema = z.object({
  principal: z.string(),
});

type SelectPrincipalSchemaType = z.infer<typeof SelectPrincipalSchema>;

export default function SelectPrincipalForm({
  principal,
}: SelectPrincipalFormProps) {
  const navigate = useNavigate();

  const form = useForm<SelectPrincipalSchemaType>({
    resolver: zodResolver(SelectPrincipalSchema),
    defaultValues: {
      principal,
    },
  });

  async function onSubmit(values: SelectPrincipalSchemaType) {
    let p: Principal;
    try {
      p = Principal.fromText(values.principal);
    } catch (error) {
      return form.setError("principal", {
        message: "Invalid principal address",
      });
    }

    navigate({
      to: "/receive",
      search: {
        principal: p.toText(),
      },
    });
  }

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className="w-full space-y-5">
        <div className="flex flex-col w-full p-5 space-y-3 border rounded-lg">
          <FormField
            control={form.control}
            name="principal"
            render={({ field }) => (
              <FormItem>
                <FormLabel>To</FormLabel>
                <FormControl>
                  <Input placeholder="p6s35-k6zg4..." {...field} />
                </FormControl>
                <FormDescription>
                  Enter the principal address you would like to monitor.
                </FormDescription>
                <FormMessage />
              </FormItem>
            )}
          />
        </div>

        <Button type="submit" className="w-full">
          Monitor this principal
        </Button>
      </form>
    </Form>
  );
}
