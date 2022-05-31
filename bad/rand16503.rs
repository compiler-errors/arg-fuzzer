
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo16503(_: S0, _: S6, _: S7, _: S1, _: S1) {}
        
        fn test16503() { foo16503(S4, S4, S0, S6, S7, S2); }
    