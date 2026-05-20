//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta175 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk879;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk880;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk881;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta175<F: Float>(t1629: F, t4673: F, t1049: F, t1615: F, t1060: F, t381: F, t4649: F, t1022: F, t1932: F, t360: F, t1625: F, t383: F, t4657: F, t1003: F, t1058: F, t1061: F, t1063: F, t1610: F, t1630: F, t1632: F, t3180: F, t3186: F, t3200: F, t353: F, t384: F, t4615: F, t4669: F, t1055: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4674, t4677, t4678, t4680, t4681, t4684) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk879::<F>(t1629, t4673, t1049, t1615, t1060, t381, t4649, t1022, t1932, t360);
        let (t4685, t4688, t4689, t4691, t4693) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk880::<F>(t1629, t4684, t1022, t1625, t1060, t383, t4657, t1003, t1058, t1061, t1063, t1610, t1630, t1632, t3180, t3186, t3200, t353, t384, t4615, t4669, t4674, t4678, t4681);
        let t4694 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk881::<F>(t1055, t4693);
    (t4674, t4677, t4678, t4680, t4681, t4684, t4685, t4688, t4689, t4691, t4693, t4694)
}
