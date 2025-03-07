"use client";

import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";
import Cookies from "js-cookie";

export function useAuth() {
    const [isAuthenticated, setIsAuthenticated] = useState(false);
    const [token, setToken] = useState<string | null>(null); // état pour le token
    const router = useRouter();

    useEffect(() => {
        console.log("getting token.....");
        const token = Cookies.get("auth_token"); // Récupère le token depuis les cookies
        if (token) {
            console.log("TOKEN FOUND");
            setToken(token) //changement d'état
            setIsAuthenticated(true); // Met à jour l'état selon la présence du token
        } else {
            console.log("No token");
            setIsAuthenticated(false);
        }
    }, []);

    const logout = async () => {

        try {
            //Appel API /logout
            const response = await fetch("http://127.0.0.1:8080/logout", {
                method: "POST",
                credentials: "include", //Pour s'assurer que le cookie est bien envoyé
            });

            if (response.ok) {
                Cookies.remove("auth_token"); // Supprime le token côté client
                setIsAuthenticated(false); // Met à jour l'état
                setToken(null) //Réinitialise le token
                router.push("/login");  //Redirection vers la page de connexion
            }
        } catch (error) {
            console.error("Erreur lors de la déconnexion : ", error);
        }
    };

    return { isAuthenticated, token,logout };
}
