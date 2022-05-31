
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo11761(_: S2, _: S3) {}
        
        fn test11761() { foo11761(S1, S2, S4, S6, S8); }
    