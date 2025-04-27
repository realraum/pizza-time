use leptos::prelude::*;

#[component]
pub fn Summary() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center gap-2 w-full">
            <form class="flex flex-row w-full gap-2">
                // <p>"Your name"</p>
                <input type="text" class="border-2 border-gray-300 rounded-md p-1 w-full" placeholder="Enter your name"/>
                <button type="submit" class="bg-green-500 text-white rounded-md p-2">"Submit"</button>
            </form>

            <div class="flex flex-col items-center bg-green-50 rounded p-1">
                <h2 class="text-xl">"Your selection"</h2>
                <table>
                    <tbody>
                        <tr>
                            <td>"1x"</td>
                            <td>"QuadFor"</td>
                            <td>"@ 10€"</td>
                            <td><button class="bg-red-500 text-white rounded-md p-1 ml-2">"Remove"</button></td>
                        </tr>
                        <tr>
                            <td>"1x"</td>
                            <td>"Veggi"</td>
                            <td>"@ 12€"</td>
                            <td><button class="bg-red-500 text-white rounded-md p-1 ml-2">"Remove"</button></td>
                        </tr>
                    </tbody>
                </table>
            </div>

            <div class="flex flex-col items-center bg-green-50 rounded p-1">
                <h2 class="text-xl">"Other's selection"</h2>
                <h4 class="text-lg">"Alice"</h4>
                <table>
                    <tbody>
                        <tr>
                            <td>"1x"</td>
                            <td>"QuadFor"</td>
                            <td>"@ 10€"</td>
                        </tr>
                        <tr>
                            <td>"1x"</td>
                            <td>"Veggi"</td>
                            <td>"@ 12€"</td>
                        </tr>
                    </tbody>
                </table>
                <h4 class="text-lg">"Bob"</h4>
                <table>
                    <tbody>
                        <tr>
                            <td>"1x"</td>
                            <td>"QuadFor"</td>
                            <td>"@ 10€"</td>
                        </tr>
                        <tr>
                            <td>"1x"</td>
                            <td>"Veggi"</td>
                            <td>"@ 12€"</td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    }
}
