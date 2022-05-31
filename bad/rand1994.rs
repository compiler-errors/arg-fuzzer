
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1994(_: S2, _: S5, _: S8) {}
        
        fn test1994() { foo1994(S7, S2, S6, S3, S3, S0, S3); }
    