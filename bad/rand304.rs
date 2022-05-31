
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo304(_: S1, _: S4, _: S6) {}
        
        fn test304() { foo304(S1, S3, S4, S5); }
    