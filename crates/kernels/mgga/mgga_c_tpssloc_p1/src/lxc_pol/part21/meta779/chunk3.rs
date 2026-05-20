//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2705/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2705<F: Float>(t12339: F, t6427: F, t6431: F, t12345: F, t19815: F, t3865: F, t1369: F, t1362: F, t56923: F, t1363: F, t19904: F, t3870: F, t3872: F, t3876: F, t40006: F, t40008: F, t40019: F, t40060: F, t54213: F, t54220: F, t54222: F, t54237: F, t56486: F, t820: F) -> F {
    let t57007 = t12339 * t6427;
    let t57009 = t12339 * t6431;
    let t57011 = t12345 * t6427;
    let t57019 = t12345 * t6431;
    let t57021 = t19815 * t3865;
    let t57022 = t57021 * t1369;
    let t57024 = t56923 * t1362;
    let t57030 = F::new(455.0) / F::new(324.0) * t40006 - F::new(35.0) / F::new(216.0) * t40008 + F::new(35.0) / F::new(72.0) * t40019 - F::new(7.0) / F::new(384.0) * t54213 - F::new(7.0) / F::new(288.0) * t54220 - F::new(7.0) / F::new(288.0) * t54222 - F::new(7.0) / F::new(288.0) * t54237 - F::new(35.0) / F::new(576.0) * t57007 + F::new(7.0) / F::new(576.0) * t57009 + F::new(595.0) / F::new(3456.0) * t57011 + F::new(5.0) / F::new(768.0) * t19904 * t3872 + F::new(5.0) / F::new(384.0) * t1363 * t3870 * t820 * t56486 - F::new(119.0) / F::new(3456.0) * t57019 + F::new(7.0) / F::new(576.0) * t57022 - t57024 * t1369 / F::new(384.0) - t19904 * t3876 / F::new(768.0) + F::new(595.0) / F::new(1296.0) * t40060;
    t57030
}
