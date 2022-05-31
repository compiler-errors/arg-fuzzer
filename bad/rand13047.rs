
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo13047(_: S4, _: S7) {}
        
        fn test13047() { foo13047(S6, S4, S1, S5); }
    