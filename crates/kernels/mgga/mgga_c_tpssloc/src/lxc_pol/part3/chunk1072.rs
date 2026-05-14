//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1072/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1072<F: Float>(t11668: F, t15749: F, t1725: F, t698: F, t1174: F, t1230: F, t14706: F, t248: F, t15426: F, t68: F, t484: F, t11836: F, t11839: F, t11842: F, t1227: F, t15727: F, t15731: F, t15735: F, t15737: F, t15740: F, t15745: F, t3490: F, t3511: F, t3577: F, t3580: F, t3587: F, t488: F, t5024: F, t5030: F) -> (F,) {
    let t15750 = t11668 * t15749;
    let t15753 = t698 * t1725;
    let t15754 = t1174 * t15753;
    let t15761 = t248 * t1230 * t14706;
    let t15764 = t15426 * t68;
    let t15765 = t15764 * t484;
    let t15768 = t15727 / 162.0 - t15731 / 13824.0 + t15735 / 20736.0 + t15737 * t3511 / 1536.0 - t15740 * t3580 / 2304.0 + t15745 + t11836 / 648.0 - t11839 / 864.0 - t11842 / 432.0 + 5.0 / 6912.0 * t3577 * t15750 + t15754 / 1296.0 - 5.0 / 2592.0 * t5024 * t3587 - t3490 * t5030 / 2304.0 - t1227 * t15761 / 4608.0 + t15765 * t488 / 3072.0;
    (t15768,)
}
