
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6624(_: S5, _: S8) {}
        
        fn test6624() { foo6624(S1, S4, S5, S6, S7); }
    