//! Safe wrapper functions for LDA_C_PW CubeCL kernel launches.
//!
//! Auto-generated launch wrappers. All unsafe confined to this module.

use cubecl::cpu::CpuRuntime;
use cubecl::client::ComputeClient;
use cubecl::prelude::*;

use super::lda_c_pw;
use super::launch_lda_x::BufArg;

// ============================================================================
// UNPOLARIZED WRAPPERS
// ============================================================================

#[allow(clippy::too_many_arguments)]
pub fn launch_lda_c_pw_exc_unpol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: &BufArg<'_>,
    zk: &BufArg<'_>,
    param_a: f64,
    param_alpha1: f64,
    param_beta1: f64,
    param_beta2: f64,
    param_beta3: f64,
    param_beta4: f64,
    param_fz20: f64,
    param_pp: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        lda_c_pw::lda_c_pw_exc_unpol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            ArrayArg::from_raw_parts::<f64>(rho.handle, rho.len, 1),
            ArrayArg::from_raw_parts::<f64>(zk.handle, zk.len, 1),
            ScalarArg::new(param_a),
            ScalarArg::new(param_alpha1),
            ScalarArg::new(param_beta1),
            ScalarArg::new(param_beta2),
            ScalarArg::new(param_beta3),
            ScalarArg::new(param_beta4),
            ScalarArg::new(param_fz20),
            ScalarArg::new(param_pp),
            ScalarArg::new(dens_threshold),
            ScalarArg::new(zeta_threshold),
        )?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn launch_lda_c_pw_vxc_unpol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: &BufArg<'_>,
    zk: &BufArg<'_>,
    vrho: &BufArg<'_>,
    param_a: f64,
    param_alpha1: f64,
    param_beta1: f64,
    param_beta2: f64,
    param_beta3: f64,
    param_beta4: f64,
    param_fz20: f64,
    param_pp: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        lda_c_pw::lda_c_pw_vxc_unpol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            ArrayArg::from_raw_parts::<f64>(rho.handle, rho.len, 1),
            ArrayArg::from_raw_parts::<f64>(zk.handle, zk.len, 1),
            ArrayArg::from_raw_parts::<f64>(vrho.handle, vrho.len, 1),
            ScalarArg::new(param_a),
            ScalarArg::new(param_alpha1),
            ScalarArg::new(param_beta1),
            ScalarArg::new(param_beta2),
            ScalarArg::new(param_beta3),
            ScalarArg::new(param_beta4),
            ScalarArg::new(param_fz20),
            ScalarArg::new(param_pp),
            ScalarArg::new(dens_threshold),
            ScalarArg::new(zeta_threshold),
        )?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn launch_lda_c_pw_fxc_unpol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: &BufArg<'_>,
    zk: &BufArg<'_>,
    vrho: &BufArg<'_>,
    v2rho2: &BufArg<'_>,
    param_a: f64,
    param_alpha1: f64,
    param_beta1: f64,
    param_beta2: f64,
    param_beta3: f64,
    param_beta4: f64,
    param_fz20: f64,
    param_pp: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        lda_c_pw::lda_c_pw_fxc_unpol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            ArrayArg::from_raw_parts::<f64>(rho.handle, rho.len, 1),
            ArrayArg::from_raw_parts::<f64>(zk.handle, zk.len, 1),
            ArrayArg::from_raw_parts::<f64>(vrho.handle, vrho.len, 1),
            ArrayArg::from_raw_parts::<f64>(v2rho2.handle, v2rho2.len, 1),
            ScalarArg::new(param_a),
            ScalarArg::new(param_alpha1),
            ScalarArg::new(param_beta1),
            ScalarArg::new(param_beta2),
            ScalarArg::new(param_beta3),
            ScalarArg::new(param_beta4),
            ScalarArg::new(param_fz20),
            ScalarArg::new(param_pp),
            ScalarArg::new(dens_threshold),
            ScalarArg::new(zeta_threshold),
        )?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn launch_lda_c_pw_kxc_unpol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: &BufArg<'_>,
    zk: &BufArg<'_>,
    vrho: &BufArg<'_>,
    v2rho2: &BufArg<'_>,
    v3rho3: &BufArg<'_>,
    param_a: f64,
    param_alpha1: f64,
    param_beta1: f64,
    param_beta2: f64,
    param_beta3: f64,
    param_beta4: f64,
    param_fz20: f64,
    param_pp: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        lda_c_pw::lda_c_pw_kxc_unpol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            ArrayArg::from_raw_parts::<f64>(rho.handle, rho.len, 1),
            ArrayArg::from_raw_parts::<f64>(zk.handle, zk.len, 1),
            ArrayArg::from_raw_parts::<f64>(vrho.handle, vrho.len, 1),
            ArrayArg::from_raw_parts::<f64>(v2rho2.handle, v2rho2.len, 1),
            ArrayArg::from_raw_parts::<f64>(v3rho3.handle, v3rho3.len, 1),
            ScalarArg::new(param_a),
            ScalarArg::new(param_alpha1),
            ScalarArg::new(param_beta1),
            ScalarArg::new(param_beta2),
            ScalarArg::new(param_beta3),
            ScalarArg::new(param_beta4),
            ScalarArg::new(param_fz20),
            ScalarArg::new(param_pp),
            ScalarArg::new(dens_threshold),
            ScalarArg::new(zeta_threshold),
        )?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn launch_lda_c_pw_lxc_unpol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: &BufArg<'_>,
    zk: &BufArg<'_>,
    vrho: &BufArg<'_>,
    v2rho2: &BufArg<'_>,
    v3rho3: &BufArg<'_>,
    v4rho4: &BufArg<'_>,
    param_a: f64,
    param_alpha1: f64,
    param_beta1: f64,
    param_beta2: f64,
    param_beta3: f64,
    param_beta4: f64,
    param_fz20: f64,
    param_pp: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        lda_c_pw::lda_c_pw_lxc_unpol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            ArrayArg::from_raw_parts::<f64>(rho.handle, rho.len, 1),
            ArrayArg::from_raw_parts::<f64>(zk.handle, zk.len, 1),
            ArrayArg::from_raw_parts::<f64>(vrho.handle, vrho.len, 1),
            ArrayArg::from_raw_parts::<f64>(v2rho2.handle, v2rho2.len, 1),
            ArrayArg::from_raw_parts::<f64>(v3rho3.handle, v3rho3.len, 1),
            ArrayArg::from_raw_parts::<f64>(v4rho4.handle, v4rho4.len, 1),
            ScalarArg::new(param_a),
            ScalarArg::new(param_alpha1),
            ScalarArg::new(param_beta1),
            ScalarArg::new(param_beta2),
            ScalarArg::new(param_beta3),
            ScalarArg::new(param_beta4),
            ScalarArg::new(param_fz20),
            ScalarArg::new(param_pp),
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
pub fn launch_lda_c_pw_exc_pol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: &BufArg<'_>,
    zk: &BufArg<'_>,
    param_a: f64,
    param_alpha1: f64,
    param_beta1: f64,
    param_beta2: f64,
    param_beta3: f64,
    param_beta4: f64,
    param_fz20: f64,
    param_pp: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        lda_c_pw::lda_c_pw_exc_pol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            ArrayArg::from_raw_parts::<f64>(rho.handle, rho.len, 1),
            ArrayArg::from_raw_parts::<f64>(zk.handle, zk.len, 1),
            ScalarArg::new(param_a),
            ScalarArg::new(param_alpha1),
            ScalarArg::new(param_beta1),
            ScalarArg::new(param_beta2),
            ScalarArg::new(param_beta3),
            ScalarArg::new(param_beta4),
            ScalarArg::new(param_fz20),
            ScalarArg::new(param_pp),
            ScalarArg::new(dens_threshold),
            ScalarArg::new(zeta_threshold),
        )?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn launch_lda_c_pw_vxc_pol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: &BufArg<'_>,
    zk: &BufArg<'_>,
    vrho: &BufArg<'_>,
    param_a: f64,
    param_alpha1: f64,
    param_beta1: f64,
    param_beta2: f64,
    param_beta3: f64,
    param_beta4: f64,
    param_fz20: f64,
    param_pp: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        lda_c_pw::lda_c_pw_vxc_pol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            ArrayArg::from_raw_parts::<f64>(rho.handle, rho.len, 1),
            ArrayArg::from_raw_parts::<f64>(zk.handle, zk.len, 1),
            ArrayArg::from_raw_parts::<f64>(vrho.handle, vrho.len, 1),
            ScalarArg::new(param_a),
            ScalarArg::new(param_alpha1),
            ScalarArg::new(param_beta1),
            ScalarArg::new(param_beta2),
            ScalarArg::new(param_beta3),
            ScalarArg::new(param_beta4),
            ScalarArg::new(param_fz20),
            ScalarArg::new(param_pp),
            ScalarArg::new(dens_threshold),
            ScalarArg::new(zeta_threshold),
        )?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn launch_lda_c_pw_fxc_pol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: &BufArg<'_>,
    zk: &BufArg<'_>,
    vrho: &BufArg<'_>,
    v2rho2: &BufArg<'_>,
    param_a: f64,
    param_alpha1: f64,
    param_beta1: f64,
    param_beta2: f64,
    param_beta3: f64,
    param_beta4: f64,
    param_fz20: f64,
    param_pp: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        lda_c_pw::lda_c_pw_fxc_pol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            ArrayArg::from_raw_parts::<f64>(rho.handle, rho.len, 1),
            ArrayArg::from_raw_parts::<f64>(zk.handle, zk.len, 1),
            ArrayArg::from_raw_parts::<f64>(vrho.handle, vrho.len, 1),
            ArrayArg::from_raw_parts::<f64>(v2rho2.handle, v2rho2.len, 1),
            ScalarArg::new(param_a),
            ScalarArg::new(param_alpha1),
            ScalarArg::new(param_beta1),
            ScalarArg::new(param_beta2),
            ScalarArg::new(param_beta3),
            ScalarArg::new(param_beta4),
            ScalarArg::new(param_fz20),
            ScalarArg::new(param_pp),
            ScalarArg::new(dens_threshold),
            ScalarArg::new(zeta_threshold),
        )?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn launch_lda_c_pw_kxc_pol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: &BufArg<'_>,
    zk: &BufArg<'_>,
    vrho: &BufArg<'_>,
    v2rho2: &BufArg<'_>,
    v3rho3: &BufArg<'_>,
    param_a: f64,
    param_alpha1: f64,
    param_beta1: f64,
    param_beta2: f64,
    param_beta3: f64,
    param_beta4: f64,
    param_fz20: f64,
    param_pp: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        lda_c_pw::lda_c_pw_kxc_pol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            ArrayArg::from_raw_parts::<f64>(rho.handle, rho.len, 1),
            ArrayArg::from_raw_parts::<f64>(zk.handle, zk.len, 1),
            ArrayArg::from_raw_parts::<f64>(vrho.handle, vrho.len, 1),
            ArrayArg::from_raw_parts::<f64>(v2rho2.handle, v2rho2.len, 1),
            ArrayArg::from_raw_parts::<f64>(v3rho3.handle, v3rho3.len, 1),
            ScalarArg::new(param_a),
            ScalarArg::new(param_alpha1),
            ScalarArg::new(param_beta1),
            ScalarArg::new(param_beta2),
            ScalarArg::new(param_beta3),
            ScalarArg::new(param_beta4),
            ScalarArg::new(param_fz20),
            ScalarArg::new(param_pp),
            ScalarArg::new(dens_threshold),
            ScalarArg::new(zeta_threshold),
        )?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn launch_lda_c_pw_lxc_pol(
    client: &ComputeClient<CpuRuntime>,
    cube_count: CubeCount,
    cube_dim: CubeDim,
    rho: &BufArg<'_>,
    zk: &BufArg<'_>,
    vrho: &BufArg<'_>,
    v2rho2: &BufArg<'_>,
    v3rho3: &BufArg<'_>,
    v4rho4: &BufArg<'_>,
    param_a: f64,
    param_alpha1: f64,
    param_beta1: f64,
    param_beta2: f64,
    param_beta3: f64,
    param_beta4: f64,
    param_fz20: f64,
    param_pp: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        lda_c_pw::lda_c_pw_lxc_pol::launch_unchecked::<CpuRuntime>(
            client, cube_count, cube_dim,
            ArrayArg::from_raw_parts::<f64>(rho.handle, rho.len, 1),
            ArrayArg::from_raw_parts::<f64>(zk.handle, zk.len, 1),
            ArrayArg::from_raw_parts::<f64>(vrho.handle, vrho.len, 1),
            ArrayArg::from_raw_parts::<f64>(v2rho2.handle, v2rho2.len, 1),
            ArrayArg::from_raw_parts::<f64>(v3rho3.handle, v3rho3.len, 1),
            ArrayArg::from_raw_parts::<f64>(v4rho4.handle, v4rho4.len, 1),
            ScalarArg::new(param_a),
            ScalarArg::new(param_alpha1),
            ScalarArg::new(param_beta1),
            ScalarArg::new(param_beta2),
            ScalarArg::new(param_beta3),
            ScalarArg::new(param_beta4),
            ScalarArg::new(param_fz20),
            ScalarArg::new(param_pp),
            ScalarArg::new(dens_threshold),
            ScalarArg::new(zeta_threshold),
        )?;
    }
    Ok(())
}
