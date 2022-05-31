
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo136(_: S4, _: S5) {}
        
        fn test136() { foo136(S8, S4, S2, S3); }
    