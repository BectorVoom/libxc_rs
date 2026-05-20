//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2208/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2208<F: Float>(t109: F, t2332: F, t4043: F, t4067: F, t45421: F, t45422: F, t45426: F, t45432: F, t45656: F, t45659: F, t45660: F, t45662: F, t45780: F, t64: F, t9365: F, t9411: F) -> F {
    let t110 = F::new(1.0) < t109;
    let t45782 = piecewise3::<F>(t110, F::new(0.0), t45421 + F::new(154.0) / F::new(9.0) * t45422 - F::new(11.0) / F::new(3.0) * t45426 + t45432 / F::new(3.0) + t64 * t4043 * t9411 / F::new(4.0) + F::new(154.0) / F::new(27.0) * t45656 + t45659 - F::new(4.0) * t45660 - F::new(2.0) * t45662 - F::new(9.0) / F::new(4.0) * t64 * t9365 * t4067 * t2332 + t45780);
    t45782
}
