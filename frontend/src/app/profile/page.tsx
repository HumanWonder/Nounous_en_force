"use client";
import { useState, useEffect } from "react";
import { useRouter } from "next/navigation";
import { useAuth } from "../hooks/useAuth";


// Déclare un type pour les données de l'utilisateur, sinon TypeScript panique en voyant null
type UserData = {
    email: string;
    role: string; 
};

export default function Profile() {
    const router = useRouter();
    const {token, isAuthenticated} = useAuth();
    const [userData, setUserData] = useState<UserData | null>(null);
    const [message, setMessage] = useState("");
    const [loading, setLoading] = useState(true); // Ajout d'un état de chargement

    useEffect(() => {
        const fetchUserProfile = async () => {
            // // Récupérer le token depuis les cookies (remarque: ajuster selon ton outil de gestion de cookies)
            console.log("Token in profile : ",token)
            if (!token || !isAuthenticated) {
                setMessage("Token inexistant, connexion non authentifiée");
                // setTimeout(() => router.push("/login"), 5000); // Redirige vers la page de login si aucun token
                return;
            }

            try {
                const response = await fetch("http://127.0.0.1:8080/profile", {
                    method: "GET",
                    headers: {
                        "Authorization": `Bearer ${token}`,  // Ajouter le token dans l'en-tête
                    },
                    credentials: "include",
                });

                const data = await response.json();
                console.log(data);

                if (response.ok) {
                    setUserData(data);
                } else {
                    setMessage(data.message || "Erreur de récupération des données. Veuillez vous reconnecter.");

                    // setTimeout(() => router.push("/login"), 5000); // Redirige vers la page de login si le token est invalide
                }
            } catch (error) {
                console.error("Erreur lors de la récupération du profil:", error);
                setMessage("Erreur réseau. Vérifiez votre connexion.");
            } finally {
                setLoading(false); // Marque le chargement comme terminé
            }
        };
        
        if (token && isAuthenticated) {
            fetchUserProfile();
        } else {
            setLoading(false); // Marque aussi comme terminé si le token n'est pas présent
        }
    }, [isAuthenticated, token, router]);

    if (loading) {
        return <p>Chargement des données...</p>; // Affiche un message de chargement
    }

    return (
        <div>
            {message && <p>{message}</p>}
            {userData ? (
                <div>
                    <h2>Profil de l'utilisateur</h2>
                    <p>Email : {userData[1]}</p>
                    <p>{userData[2]}</p>
                    {/* Affiche d'autres données utilisateur ici */}
                </div>
            ) : (
                <p>Profil non trouvé</p>
            )}
        </div>
    );
}
