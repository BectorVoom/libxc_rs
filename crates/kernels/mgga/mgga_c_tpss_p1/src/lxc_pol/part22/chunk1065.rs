//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1065/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1065<F: Float>(t11648: F, t2741: F, t1407: F, t2725: F, t2682: F, t3941: F, t11631: F, t11637: F, t11641: F, t11647: F, t1467: F, t2685: F, t2722: F, t2740: F, t3928: F, t3956: F, t8509: F, t8514: F, t8958: F, t8976: F, t9042: F) -> F {
    let t11649 = t2741 * t11648;
    let t11652 = t1407 * t2725;
    let t11653 = t2741 * t11652;
    let t11659 = t2682 * t3941 / F::new(432.0);
    let t11660 = -t8509 * t11631 / F::new(4608.0) + t8976 * t3956 / F::new(288.0) + t2722 * t11637 / F::new(768.0) - t11641 / F::new(1296.0) - t2685 * t3928 / F::new(54.0) + t11647 + t9042 - t2740 * t11649 / F::new(2304.0) + t8514 * t11653 / F::new(2304.0) + F::new(19.0) / F::new(1728.0) * t8958 * t1467 - t11659;
    t11660
}
