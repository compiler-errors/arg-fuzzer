
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo14187(_: S4, _: S7) {}
        
        fn test14187() { foo14187(S2, S4, S8); }
    