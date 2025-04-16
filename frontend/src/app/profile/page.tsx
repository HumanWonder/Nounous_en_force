"use client";
import { useState, useEffect } from "react";
import { useRouter } from "next/navigation";
import { useAuth } from "../hooks/useAuth";

import { Accordion } from "@/components/ui/Accordion";
import type { FullProfileData } from "../types/user";

export default function Profile() {
    const router = useRouter();
    const { token, isAuthenticated } = useAuth();
    const [profileData, setProfileData] = useState<FullProfileData | null>(null);
    const [message, setMessage] = useState("");
    const [loading, setLoading] = useState(true); // Ajout d'un état de chargement

    useEffect(() => {
        const fetchUserProfile = async () => {
            // // Récupérer le token depuis les cookies (ajuster selon outil de gestion de cookies)
            console.log("Token in profile : ", token)
            if (!token || !isAuthenticated) {
                setMessage("Token inexistant, connexion non authentifiée");
                // setTimeout(() => router.push("/login"), 5000); // Redirige vers la page de login si aucun token
                return;
            }
            console.log("Token:", token);
            console.log("isAuthenticated:", isAuthenticated);

            try {
                const response = await fetch("http://127.0.0.1:8080/profile", {
                    method: "GET",
                    headers: {
                        "Authorization": `Bearer ${token}`,  // Ajouter le token dans l'en-tête
                    },
                    credentials: "include",
                });

                const data = await response.json();
                console.log("data :", data);
                // setProfileData(data);

                if (response.ok) {

                    setProfileData({
                        user: data.user,
                        temp_data: data.user.role === "temp" ? {
                            temp_info: data.temp.temp,
                            availabilities: data.temp.availabilities,
                            diplomas: data.temp.diplomas,
                            experiences: data.temp.experiences,
                            conditions: data.temp.conditions,
                            skills: data.temp.skills,
                            documents: data.temp.documents,
                        } : undefined,
                        owner_data: data.user.role === "owner" ? {
                            owner: data.owner,
                            creches: [],
                        } : undefined,
                    });

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

    useEffect(() => {
        console.log("profileData :", profileData);
    }, [profileData]);


    return (
        <div>
            {message && <p>{message}</p>}
            {profileData ? (
                <div>
                    <h2 className="text-2xl font-bold mb-4">Mon profil {profileData.user.role}</h2>
                    <br />
                    <p>Email : {profileData.user.email}</p>
                    <p>Rôle : {profileData.user.role}</p>
                    <br />
                    {profileData.user.role === "pending" && (
                        <button className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-full"
                            onClick={() => router.push("register/complete/")}>
                            Compléter mon inscription
                        </button>
                    )}

                    {profileData.user.role === "admin" && (
                        <button className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-full"
                            onClick={() => router.push("/admin")}>
                            Page Admin
                        </button>
                    )}
                    {/* Affiche d'autres données utilisateur ici */}
                    {profileData.user.role === "temp" && profileData.temp_data ? (
                        <>
                            <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 p-6">
                                {/* Colonne gauche */}
                                <div className="bg-white p-4 rounded-xl shadow space-y-2">
                                    <div className="space-y-1">
                                        <p><strong>Nom :</strong> {profileData.temp_data.temp_info.last_name}</p>
                                        <p><strong>Prénom :</strong> {profileData.temp_data.temp_info.first_name}</p>
                                        <p><strong>Adresse :</strong> {profileData.temp_data.temp_info.address}</p>
                                        <p><strong>Téléphone :</strong> {profileData.temp_data.temp_info.phone}</p>
                                        <p><strong>Date de naissance :</strong> {profileData.temp_data.temp_info.birth_date}</p>
                                        <p><strong>Permis :</strong> {profileData.temp_data.temp_info.has_driver_license ? "Oui" : "Non"}</p>
                                        <p><strong>Transport :</strong> {profileData.temp_data.temp_info.transport_mode}</p>
                                    </div>
                                </div>

                                {/* Colonne droite */}
                                <div className="space-y-4">
                                    <Accordion title="Disponibilités">
                                        {profileData.temp_data.availabilities.map((a, i) => (
                                            <div key={i} className="mb-2">
                                                <p>Périodes : {a.availability_periods}</p>
                                                <p>Horaires : {a.time_slots}</p>
                                                <p>Zones : {a.geographic_zones}</p>
                                                <p>Trajet max : {a.max_travel_time}</p>
                                            </div>
                                        ))}
                                    </Accordion>

                                    <Accordion title="Conditions de travail">
                                        {profileData.temp_data.conditions?.map((c, i) => (
                                            <div key={i}>
                                                <p>Taux horaire : {c.hourly_rate}€</p>
                                                <p>Contrats : {c.contract_types}</p>
                                                <p>Auto-entrepreneur : {c.auto_entrepreneur ? "Oui" : "Non"}</p>
                                            </div>
                                        ))}
                                    </Accordion>

                                    <Accordion title="Diplômes">
                                        {profileData.temp_data.diplomas?.map((d, i) => (
                                            <div key={i}>
                                                <p>Diplôme : {d.main_diploma}</p>
                                                <p>Autres certifications : {d.other_certifications}</p>
                                                <p>Année : {d.graduation_year}</p>
                                                <p>Établissement : {d.school}</p>
                                            </div>
                                        ))}
                                    </Accordion>

                                    <Accordion title="Expériences professionnelles">
                                        {profileData.temp_data.experiences?.map((e, i) => (
                                            <div key={i}>
                                                <p>Expérience : {e.total_experience}</p>
                                                <p>Postes : {e.previous_positions}</p>
                                                <p>Structures : {e.structure_types}</p>
                                                <p>Tâches : {e.tasks}</p>
                                            </div>
                                        ))}
                                    </Accordion>

                                    <Accordion title="Compétences">
                                        {profileData.temp_data.skills?.map((s, i) => (
                                            <div key={i}>
                                                <p>Langues : {s.languages}</p>
                                                <p>Pédagogies : {s.pedagogies}</p>
                                                <p>Compétences : {s.special_skills}</p>
                                                <p>Prise en charge : {s.special_needs_handling}</p>
                                            </div>
                                        ))}
                                    </Accordion>

                                    <Accordion title="Documents">
                                        {profileData.temp_data.documents?.map((doc, i) => (
                                            <div key={i}>
                                                <p>Lettre de motivation : {doc.motivation_letter}</p>
                                                <p>Casier judiciaire : {doc.criminal_record}</p>
                                                <p>Identité : {doc.required_documents}</p>
                                                <p>Diplômes : {doc.diplomas}</p>
                                                <p>Références : {doc.professional_references}</p>
                                            </div>
                                        ))}
                                    </Accordion>
                                </div>
                            </div>
                        </>
                    ) : null}
                </div>
            ) : (
                <p>Données indisponibles...</p>
            )}
        </div>
    );
}
