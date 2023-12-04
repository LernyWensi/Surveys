import { useEffect, useState } from "react";
import { Home } from "./components/Home/Home";
import { SurveySearchBar } from "./components/SurveySearchBar/SurveySearchBar";
import { Welcome } from "./components/Welcome/Welcome";

export interface User {
    id: string;
    token: string;
}

const App = () => {
    const [user, setUser] = useState<User | null>(null);

    const setUserHandler = (user: User | null) => {
        setUser(user);

        if (user) {
            localStorage.setItem("user", JSON.stringify(user));
            return;
        }

        localStorage.removeItem("user");
    };

    useEffect(() => {
        const localStorageUser = localStorage.getItem("user");
        localStorageUser && setUser(JSON.parse(localStorageUser));
    }, []);

    return (
        <>
            <SurveySearchBar />
            {user ? <Home setUserHandler={setUserHandler} /> : <Welcome setUserHandler={setUserHandler} />}
        </>
    );
};

export default App;
