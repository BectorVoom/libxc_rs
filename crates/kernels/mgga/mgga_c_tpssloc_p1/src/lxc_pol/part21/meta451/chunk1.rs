//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2004/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2004<F: Float>(t15426: F, t68: F, t484: F, t11836: F, t11839: F, t11842: F, t1227: F, t15727: F, t15731: F, t15735: F, t15737: F, t15740: F, t15745: F, t15750: F, t15754: F, t15761: F, t3490: F, t3511: F, t3577: F, t3580: F, t3587: F, t488: F, t5024: F, t5030: F) -> (F, F, F) {
    let t15764 = t15426 * t68;
    let t15765 = t15764 * t484;
    let t15768 = t15727 / F::new(162.0) - t15731 / F::new(13824.0) + t15735 / F::new(20736.0) + t15737 * t3511 / F::new(1536.0) - t15740 * t3580 / F::new(2304.0) + t15745 + t11836 / F::new(648.0) - t11839 / F::new(864.0) - t11842 / F::new(432.0) + F::new(5.0) / F::new(6912.0) * t3577 * t15750 + t15754 / F::new(1296.0) - F::new(5.0) / F::new(2592.0) * t5024 * t3587 - t3490 * t5030 / F::new(2304.0) - t1227 * t15761 / F::new(4608.0) + t15765 * t488 / F::new(3072.0);
    (t15764, t15765, t15768)
}
