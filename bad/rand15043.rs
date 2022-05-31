
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo15043(_: S3, _: S6, _: S5, _: S6) {}
        
        fn test15043() { foo15043(S1, S5, S0, S4, S3, S4); }
    