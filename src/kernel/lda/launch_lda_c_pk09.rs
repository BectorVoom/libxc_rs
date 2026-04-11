//! Safe wrapper functions for LDA_C_PK09 CubeCL kernel launches.
//!
//! Auto-generated launch wrappers. All unsafe confined to this module.

use cubecl::cpu::CpuRuntime;
use cubecl::client::ComputeClient;
use cubecl::prelude::*;

use super::lda_c_pk09;
use super::launch_lda_x::BufArg;

// ============================================================================
// UNPOLARIZED WRAPPERS
// ============================================================================

#[allow(clippy::too_many_arguments)]
pub fn launch_lda_c_pk09_exc_unpol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: &BufArg<'_>,
    zk: &BufArg<'_>,
    dens_threshold: f64,
    zeta_threshold: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        lda_c_pk09::lda_c_pk09_exc_unpol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            ArrayArg::from_raw_parts::<f64>(rho.handle, rho.len, 1),
            ArrayArg::from_raw_parts::<f64>(zk.handle, zk.len, 1),
            ScalarArg::new(dens_threshold),
            ScalarArg::new(zeta_threshold),
        )?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn launch_lda_c_pk09_vxc_unpol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: &BufArg<'_>,
    zk: &BufArg<'_>,
    vrho: &BufArg<'_>,
    dens_threshold: f64,
    zeta_threshold: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        lda_c_pk09::lda_c_pk09_vxc_unpol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            ArrayArg::from_raw_parts::<f64>(rho.handle, rho.len, 1),
            ArrayArg::from_raw_parts::<f64>(zk.handle, zk.len, 1),
            ArrayArg::from_raw_parts::<f64>(vrho.handle, vrho.len, 1),
            ScalarArg::new(dens_threshold),
            ScalarArg::new(zeta_threshold),
        )?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn launch_lda_c_pk09_fxc_unpol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: &BufArg<'_>,
    zk: &BufArg<'_>,
    vrho: &BufArg<'_>,
    v2rho2: &BufArg<'_>,
    dens_threshold: f64,
    zeta_threshold: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        lda_c_pk09::lda_c_pk09_fxc_unpol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            ArrayArg::from_raw_parts::<f64>(rho.handle, rho.len, 1),
            ArrayArg::from_raw_parts::<f64>(zk.handle, zk.len, 1),
            ArrayArg::from_raw_parts::<f64>(vrho.handle, vrho.len, 1),
            ArrayArg::from_raw_parts::<f64>(v2rho2.handle, v2rho2.len, 1),
            ScalarArg::new(dens_threshold),
            ScalarArg::new(zeta_threshold),
        )?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn launch_lda_c_pk09_kxc_unpol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: &BufArg<'_>,
    zk: &BufArg<'_>,
    vrho: &BufArg<'_>,
    v2rho2: &BufArg<'_>,
    v3rho3: &BufArg<'_>,
    dens_threshold: f64,
    zeta_threshold: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        lda_c_pk09::lda_c_pk09_kxc_unpol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            ArrayArg::from_raw_parts::<f64>(rho.handle, rho.len, 1),
            ArrayArg::from_raw_parts::<f64>(zk.handle, zk.len, 1),
            ArrayArg::from_raw_parts::<f64>(vrho.handle, vrho.len, 1),
            ArrayArg::from_raw_parts::<f64>(v2rho2.handle, v2rho2.len, 1),
            ArrayArg::from_raw_parts::<f64>(v3rho3.handle, v3rho3.len, 1),
            ScalarArg::new(dens_threshold),
            ScalarArg::new(zeta_threshold),
        )?;
    }
    Ok(())
}

// ============================================================================
// POLARIZED WRAPPERS
// ============================================================================

#[allow(clippy::too_many_arguments)]
pub fn launch_lda_c_pk09_exc_pol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: &BufArg<'_>,
    zk: &BufArg<'_>,
    dens_threshold: f64,
    zeta_threshold: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        lda_c_pk09::lda_c_pk09_exc_pol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            ArrayArg::from_raw_parts::<f64>(rho.handle, rho.len, 1),
            ArrayArg::from_raw_parts::<f64>(zk.handle, zk.len, 1),
            ScalarArg::new(dens_threshold),
            ScalarArg::new(zeta_threshold),
        )?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn launch_lda_c_pk09_vxc_pol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: &BufArg<'_>,
    zk: &BufArg<'_>,
    vrho: &BufArg<'_>,
    dens_threshold: f64,
    zeta_threshold: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        lda_c_pk09::lda_c_pk09_vxc_pol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            ArrayArg::from_raw_parts::<f64>(rho.handle, rho.len, 1),
            ArrayArg::from_raw_parts::<f64>(zk.handle, zk.len, 1),
            ArrayArg::from_raw_parts::<f64>(vrho.handle, vrho.len, 1),
            ScalarArg::new(dens_threshold),
            ScalarArg::new(zeta_threshold),
        )?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn launch_lda_c_pk09_fxc_pol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: &BufArg<'_>,
    zk: &BufArg<'_>,
    vrho: &BufArg<'_>,
    v2rho2: &BufArg<'_>,
    dens_threshold: f64,
    zeta_threshold: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        lda_c_pk09::lda_c_pk09_fxc_pol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            ArrayArg::from_raw_parts::<f64>(rho.handle, rho.len, 1),
            ArrayArg::from_raw_parts::<f64>(zk.handle, zk.len, 1),
            ArrayArg::from_raw_parts::<f64>(vrho.handle, vrho.len, 1),
            ArrayArg::from_raw_parts::<f64>(v2rho2.handle, v2rho2.len, 1),
            ScalarArg::new(dens_threshold),
            ScalarArg::new(zeta_threshold),
        )?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn launch_lda_c_pk09_kxc_pol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: &BufArg<'_>,
    zk: &BufArg<'_>,
    vrho: &BufArg<'_>,
    v2rho2: &BufArg<'_>,
    v3rho3: &BufArg<'_>,
    dens_threshold: f64,
    zeta_threshold: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        lda_c_pk09::lda_c_pk09_kxc_pol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            ArrayArg::from_raw_parts::<f64>(rho.handle, rho.len, 1),
            ArrayArg::from_raw_parts::<f64>(zk.handle, zk.len, 1),
            ArrayArg::from_raw_parts::<f64>(vrho.handle, vrho.len, 1),
            ArrayArg::from_raw_parts::<f64>(v2rho2.handle, v2rho2.len, 1),
            ArrayArg::from_raw_parts::<f64>(v3rho3.handle, v3rho3.len, 1),
            ScalarArg::new(dens_threshold),
            ScalarArg::new(zeta_threshold),
        )?;
    }
    Ok(())
}
