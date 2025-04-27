use leptos::prelude::*;

#[component]
pub fn Money() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center gap-2 w-full">
            <table>
                <tbody>
                    <tr>
                        <td>"You paid"</td>
                        <td>"10€"</td>
                    </tr>
                    <tr>
                        <td>"Your orders cost"</td>
                        <td>"8€"</td>
                    </tr>
                    <tr>
                        <td>"Current delta"</td>
                        <td>"2€"</td>
                    </tr>
                </tbody>
            </table>
            <form class="flex flex-row gap-2">
                <input type="text" class="border-2 border-gray-300 rounded-md p-1" placeholder="Update paid amount"/>
                <button type="submit" class="bg-green-500 text-white rounded-md p-2">"Submit"</button>
            </form>

            <h2 class="text-xl">"Other's balance"</h2>
            <table class="w-full">
                <thead>
                    <tr>
                        <th>"Name"</th>
                        <th>"Paid"</th>
                        <th>"Order"</th>
                        <th>"Delta"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>"Alice"</td>
                        <td>"10€"</td>
                        <td>"8€"</td>
                        <td>"2€"</td>
                    </tr>
                    <tr>
                        <td>"Bob"</td>
                        <td>"10€"</td>
                        <td>"8€"</td>
                        <td>"2€"</td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}
