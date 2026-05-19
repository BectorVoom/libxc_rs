//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1400/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1400<F: Float>(t101551: F, t114865: F, t114882: F, t114892: F, t118847: F, t118850: F, t118851: F, t121511: F, t121531: F, t121552: F, t121614: F, t1912: F, t24297: F, t25168: F, t25330: F, t26680: F, t2713: F, t33399: F, t6627: F, t6631: F, t7087: F, t7538: F, t855: F, t858: F, t92386: F) -> F {
    let t121623 = -t6627 * t26680 - t7087 * t25330 - F::new(6.0) * t25168 * t101551 * t6631 + t118847 - t855 * t858 * (t121511 + t121531 + t121552 + t121614) - t118850 - t114865 - t24297 * t7538 - t2713 * t33399 + F::cast_from(0.19190897446562641759e-1_f64) * t114882 - t118851 + t114892 - t92386 * t1912;
    t121623
}
