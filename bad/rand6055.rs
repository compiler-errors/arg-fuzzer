
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6055(_: S2, _: S3, _: S4, _: S6) {}
        
        fn test6055() { foo6055(S2, S7, S3, S5, S1, S1); }
    