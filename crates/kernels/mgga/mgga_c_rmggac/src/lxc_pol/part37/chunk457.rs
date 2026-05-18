//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 457/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk457<F: Float>(t118: F, t305: F, t8889: F, t8891: F, t8893: F, t8895: F, t8897: F, t8899: F, t8903: F, t8907: F, t8909: F, t8911: F, t8913: F, t8917: F, t9427: F, t9437: F) -> F {
    let t9518 = -F::new(0.40911992481368012596e-1) * t8889 + F::new(0.81823984962736025192e-1) * t8891 + F::new(0.20455996240684006298e-1) * t8893 + F::new(0.8182398496273602519e-1) * t8895 - F::new(0.13637330827122670865e0) * t8897 - F::new(0.2727466165424534173e-1) * t8899 + F::new(0.20455996240684006298e-1) * t8903 - F::new(0.2727466165424534173e-1) * t8907 - F::new(0.13637330827122670865e-1) * t8909 + F::new(0.59871208509319042821e-1) * t305 * t9437 - F::new(0.39914139006212695214e-1) * t118 * t9427 + F::new(0.54549323308490683461e-1) * t8911 - F::new(0.72732431077987577947e-1) * t8913 - F::new(0.18183107769496894487e-1) * t8917;
    t9518
}
