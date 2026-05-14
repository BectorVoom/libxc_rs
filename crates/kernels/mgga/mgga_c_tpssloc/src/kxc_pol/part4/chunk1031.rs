//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1031/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1031<F: Float>(t3270: F, t5999: F, t1102: F, t11137: F, t14818: F, t18227: F, t18239: F, t18497: F, t18500: F, t18503: F, t18508: F, t18510: F, t18515: F, t18518: F, t11369: F, t14722: F, t14766: F, t14768: F, t14782: F, t18203: F, t18208: F, t18213: F, t18217: F, t18219: F, t18223: F, t18229: F, t18234: F, t18243: F, t18494: F, t18505: F, t18512: F, t18521: F, t18731: F, t18759: F) -> (F, F) {
    let t18761 = t3270 * t5999;
    let t18762 = t18761 * t1102;
    let t18783 = 0.12077e1 * t18227 + 0.36793333333333333333e-1 * t14818 - 0.27595e-1 * t18515 + 0.36793333333333333333e-1 * t18497 + 0.16557e0 * t18518 + 0.13418888888888888889e0 * t11137 + 0.60385e0 * t18239 - 0.5519e-1 * t18503 - 0.16557e0 * t18500 + 0.33114e0 * t18510 + 0.49671e0 * t18508;
    let t18785 = 0.258925e1 * t18731 - t11369 - 0.5519e-1 * t18512 + 0.82785e-1 * t18521 + 0.67094444444444444443e-1 * t18203 - 0.20128333333333333333e0 * t18219 - 0.10064166666666666667e0 * t18229 + 0.301925e0 * t18243 + 0.18396666666666666667e-1 * t18494 - 0.11038e0 * t18505 + t18759 - 0.1294625e1 * t18762 + 0.18396666666666666667e0 * t14766 + t14768 - 0.40256666666666666668e0 * t14722 - t14782 - 0.20128333333333333333e0 * t18234 + 0.33547222222222222222e0 * t18208 - 0.12077e1 * t18213 - 0.40256666666666666666e0 * t18217 + 0.181155e1 * t18223 + t18783;
    (t18762, t18785)
}
