use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct HouRect {
    pub size: Vec2,
    pub translation: Vec3,
    pub uv: Vec<Vec2>,
}

impl HouRect {
    pub fn get_halfsize(&self) -> Vec2 {
        self.size * 0.5
    }

    pub fn get_vertices(&self) -> Vec<Vec3> {
        let half_size = self.get_halfsize();

        let vertices: Vec<Vec3> = vec![
            // Vertex 0: Top-left (matches UV[0])
            Vec3::new(
                self.translation.x - half_size.x,
                self.translation.y + half_size.y,
                self.translation.z,
            ),
            // Vertex 1: Top-right (matches UV[1])
            Vec3::new(
                self.translation.x + half_size.x,
                self.translation.y + half_size.y,
                self.translation.z,
            ),
            // Vertex 2: Bottom-right (matches UV[2])
            Vec3::new(
                self.translation.x + half_size.x,
                self.translation.y - half_size.y,
                self.translation.z,
            ),
            // Vertex 3: Bottom-left (matches UV[3])
            Vec3::new(
                self.translation.x - half_size.x,
                self.translation.y - half_size.y,
                self.translation.z,
            ),
        ];

        return vertices;
    }

    pub fn get_uvs(&self) -> Vec<Vec2> {
        let uvs = if self.uv.is_empty() {
            vec![
                Vec2::new(0.0, 0.0), // Top-left becomes (0,0) after Y inversion
                Vec2::new(1.0, 0.0), // Top-right becomes (1,0) after Y inversion
                Vec2::new(1.0, 1.0), // Bottom-right becomes (1,1) after Y inversion
                Vec2::new(0.0, 1.0), // Bottom-left becomes (0,1) after Y inversion
            ]
        } else {
            // Invert Y coordinate: v' = 1.0 - v
            // UVs are in Houdini order: [TL, TR, BR, BL]
            vec![
                Vec2::new(self.uv[0].x, 1.0 - self.uv[0].y), // TL
                Vec2::new(self.uv[1].x, 1.0 - self.uv[1].y), // TR
                Vec2::new(self.uv[2].x, 1.0 - self.uv[2].y), // BR
                Vec2::new(self.uv[3].x, 1.0 - self.uv[3].y), // BL
            ]
        };

        return uvs;
    }

    pub fn get_indices(&self) -> Vec<u32> {
        return vec![0, 1, 2, 2, 3, 0];
    }
}

impl Primitive2d for HouRect {}

impl Default for HouRect {
    fn default() -> Self {
        Self {
            size: Vec2::splat(0.5),
            translation: Vec3::splat(0.),
            uv: Vec::new(),
        }
    }
}

impl MeshBuilder for HouRect {
    fn build(&self) -> Mesh {
        let vertices = self.get_vertices();
        let uvs = self.get_uvs();
        let indices = self.get_indices();

        // Create normals (all pointing forward for a 2D rectangle)
        let normals: Vec<Vec3> = vec![
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, 1.0),
        ];

        // Build the mesh
        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        );

        // Insert attributes
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_indices(Indices::U32(indices));

        mesh
    }
}

impl From<&HouRect> for Mesh {
    fn from(rect: &HouRect) -> Self {
        rect.build()
    }
}

// Helper methods
impl HouRect {
    pub fn into_mesh(self) -> Mesh {
        self.into()
    }

    pub fn to_mesh(&self) -> Mesh {
        self.build()
    }
}
