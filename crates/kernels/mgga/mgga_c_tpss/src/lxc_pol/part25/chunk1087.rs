//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1087/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1087<F: Float>(t14991: F, t2740: F, t14473: F, t3923: F, t11508: F, t11524: F, t11528: F, t11550: F, t11562: F, t14956: F, t14960: F, t14965: F, t14970: F, t14975: F, t14980: F, t14987: F, t2682: F, t2685: F, t4966: F, t4970: F, t4974: F, t4985: F, t4991: F, t8509: F, t8954: F, t8989: F, t925: F) -> F {
    let t14992 = t2740 * t14991;
    let t14994 = t3923 * t14473;
    let t14997 = -t8954 / F::new(20736.0) + t925 * t14956 / F::new(288.0) - t14960 / F::new(432.0) - t2685 * t4974 / F::new(108.0) + t14965 / F::new(864.0) - t8989 * t4985 / F::new(432.0) + F::new(5.0) / F::new(6912.0) * t2740 * t14970 - t8509 * t14975 / F::new(2304.0) + t11508 + t14980 / F::new(4608.0) - t2682 * t4991 / F::new(576.0) - t11524 + t11528 + t11550 - t2685 * t4966 / F::new(81.0) + t14987 / F::new(648.0) + t2685 * t4970 / F::new(54.0) - t11562 + t14992 / F::new(3456.0) + t925 * t14994 / F::new(48.0);
    t14997
}
