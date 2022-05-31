
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo13994(_: S7, _: S6, _: S5, _: S2, _: S5) {}
        
        fn test13994() { foo13994(S1, S3, S5, S6, S7, S8); }
    