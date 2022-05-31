
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6129(_: S4, _: S8, _: S1, _: S3) {}
        
        fn test6129() { foo6129(S5, S8, S5, S6, S1, S8, S7); }
    