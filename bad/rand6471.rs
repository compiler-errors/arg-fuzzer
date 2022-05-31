
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6471(_: S4, _: S3, _: S1, _: S6) {}
        
        fn test6471() { foo6471(S2, S3, S4, S6, S8); }
    