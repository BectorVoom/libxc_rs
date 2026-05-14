//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1270/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1270<F: Float>(t1052: F, t1055: F, t1058: F, t1060: F, t11046: F, t11059: F, t11060: F, t11065: F, t11066: F, t14608: F, t14618: F, t1603: F, t1625: F, t1635: F, t17575: F, t17588: F, t18074: F, t18086: F, t21480: F, t21614: F, t21617: F, t21618: F, t21622: F, t21623: F, t21635: F, t21638: F, t21657: F, t21663: F, t21677: F, t21692: F, t3200: F, t3201: F, t349: F, t353: F, t360: F, t381: F, t383: F, t384: F, t388: F, t43515: F, t43516: F, t43576: F, t43577: F, t4557: F, t4660: F, t4669: F, t47853: F, t5866: F, t5903: F, t5914: F, t5920: F, t5928: F, t5929: F, t5933: F, t5936: F, t5939: F, t5941: F, t5944: F, t63004: F, t63183: F, t70987: F, t76976: F, t76977: F, t77764: F, t77782: F, t77794: F, t77826: F, t77835: F, t77855: F, t77892: F) -> (F,) {
    let t77913 = 4.0 * t21480 * t1625 * t388 + t349 * t77764 * t388 + 4.0 * t1603 * t21614 * t388 + 12.0 * t17575 * t5920 - t1052 * t1055 * (6.0 * t1058 * t5914 * t5866 * t1060 + 6.0 * t11046 * t5936 * t77794 * t360 + 36.0 * t11059 * t5928 * t11060 * t5866 - 24.0 * t11065 * t77782 * t11066 + t353 * t383 * t77764 - 12.0 * t14608 * t21623 + 24.0 * t14618 * t21657 + 12.0 * t18086 * t5933 + 12.0 * t63183 * t5929 - 6.0 * t63004 * t5939 + t77835 + 24.0 * t11059 * t77782 * t11060 - 12.0 * t3200 * t21617 * t21622 - 3.0 * t3200 * t77855 * t3201 + 14.0 * t43515 * t77826 * t43516 + 24.0 * t43576 * t77826 * t43577 + 12.0 * t4669 * t21618 + 4.0 * t4669 * t21635 + 4.0 * t47853 * t21638 + t76977 * t384 + 6.0 * t5903 * t5941 + t77892) + 24.0 * t4660 * t21692 - 12.0 * t17588 * t5944 - 24.0 * t4660 * t21677 - 4.0 * t4557 * t21663 - 6.0 * t17575 * t5944 + t76976 * t381 * t388 - 6.0 * t18074 * t5944 - 12.0 * t70987 * t1635;
    (t77913,)
}
