//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 984/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk984<F: Float>(t21013: F, t235: F, t20986: F, t4282: F, t4295: F, t5612: F, t1499: F, t1523: F, t1525: F, t16673: F, t20806: F, t20854: F, t20858: F, t20862: F, t20867: F, t20871: F, t20873: F, t20876: F, t20937: F, t226: F, t255: F, t4166: F, t4281: F, t4291: F, t5575: F, t5645: F, t5648: F, t5651: F, t5653: F, t5655: F, t812: F) -> (F, F, F, F) {
    let t21014 = t235 * t21013;
    let t21025 = t4282 * t20986;
    let t21028 = t4295 * t5612;
    let t21033 = F::new(3.0) * t1499 * t5655 - F::new(3.0) * t1523 * t16673 + F::new(3.0) * t1525 * t5575 - F::new(3.0) * t20806 * t812 - t20854 * t812 - F::new(6.0) * t20858 * t812 + F::new(6.0) * t20862 * t812 + F::new(6.0) * t20867 * t812 - t20871 * t812 - F::new(3.0) * t20873 * t4291 - F::new(3.0) * t20876 * t812 + t20937 * t255 + t21014 * t226 + F::new(6.0) * t21025 * t4281 - F::new(3.0) * t21028 * t812 + F::new(6.0) * t4166 * t5645 - F::new(6.0) * t4166 * t5648 - F::new(3.0) * t4166 * t5651 - F::new(3.0) * t4166 * t5653;
    (t21014, t21025, t21028, t21033)
}
