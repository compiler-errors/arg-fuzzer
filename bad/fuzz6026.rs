
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6026(_: S6, _: S5, _: S3, _: S4) {}
        
        fn test6026() { foo6026(S5, S5, S7, S3, S8, S1); }
    